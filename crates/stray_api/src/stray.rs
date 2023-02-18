use legion::*;
use stray_scene::EngineData;
use wgpu::{Surface, Device, SurfaceConfiguration};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window}, dpi::{PhysicalSize},
};

use stray_systems::*;
use stray_render::*;

use crate::settings::*;
use crate::events::*;
use crate::plugin::*;

/// ## Overview
/// Container of app data
/// 
/// Holds all the elements like Legion World,
/// Legion Resources and Legion Schedule
/// so that using them is much more easier
/// 
/// ## Example
/// 
/// Basic example that open the window
/// 
/// ```
/// use stray_api::*;
/// 
/// fn main(){
///     Stray::new()
///         .with_title("Hello World")
///         .build()
///         .run();
/// }
/// ```
/// 
pub struct Stray{
    global_schedule: Option<Schedule>,
    render_schedule: Option<Schedule>,
    global_resources: Resources,
    render_resources: Resources,
    plugins: Vec<Box<dyn Plugin>>,
    event_loop: EventLoop<()>,
    world: World,
}

impl Stray{

    /// Initialize Stray Builder For Configuration
    pub fn new() -> StrayBuilder{
        StrayBuilder::new()
    }

    /// Get non-mutable reference of global resources
    pub fn get_resources(&self) -> &Resources{
        &self.global_resources
    }

    /// Get mutable reference of global resources
    pub fn get_mut_resources(&mut self) -> &mut Resources{
        &mut self.global_resources
    }

    /// ### Starts the application by running winit event loop
    /// 
    /// Execute user and render schedule in every frame.
    /// Handle window, keyboard, mouse ***(WIP)*** and joypad ***(Planned)*** events.
    /// It also initialize render before event loop is runned
    ///
    /// ### ``Stray::run()`` might not return
    /// 
    pub fn run(mut self) {
        parse_settings(
            &self.global_resources.get::<Settings>().unwrap(), 
            &self.global_resources.get::<Window>().unwrap()
        );
        self.global_resources.insert(InputEvent::NONE);
        self.global_resources.insert(LastState::NONE);
        
        let mut r_schedule = self.render_schedule.unwrap();
        let mut g_schedule = self.global_schedule.unwrap();
        match initialize_render(
            &mut self.render_resources, 
            &self.global_resources.get::<Window>().unwrap(), 
            &self.global_resources.get::<Settings>().unwrap().backend
        ){
            Err(e) => {
                eprintln!("Render Error: {}", e);
                std::process::exit(1);
            }
            Ok(_) => {}
        }
        self.event_loop.run(move |event, _, control_flow| 
            {
                match event {
                    Event::WindowEvent {
                        ref event,
                        window_id,
                    } if window_id == self.global_resources.get::<Window>().unwrap().id() => match event {
                        WindowEvent::CursorMoved {..} => {
                            // WIP
                            // let true_position = [
                            //     position.x-(self.global_resources.get::<Window>().unwrap().inner_size().width/2) as f64, 
                            //     position.y-(self.global_resources.get::<Window>().unwrap().inner_size().height/2) as f64
                            // ];
                        }
                        WindowEvent::MouseInput {..} =>{
                            // WIP
                        }
                        WindowEvent::KeyboardInput { input, .. } => {
                            self.global_resources.insert(InputEvent::from(input));
                            self.global_resources.insert(LastState::from(input));
                        },

                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            resize(&self.render_resources, &self.global_resources, *physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            resize(&self.render_resources, &self.global_resources, **new_inner_size);
                            
                        }
                        _ => {}
                    },
                    Event::RedrawRequested(_) => {
        
                    },
                    Event::MainEventsCleared => {
                        r_schedule.execute(&mut self.world, &mut self.render_resources);
                        g_schedule.execute(&mut self.world, &mut self.global_resources);
                        self.global_resources.insert(InputEvent::NONE);
                    }
                    _ => {}
            }
    
        });
    }
}

/// ## Overview
/// Builder for stray app.
/// It contains system builders like render schedule, global schedule
/// and once schedule - which is used for initalize entites.
/// It also contain final stray struct which is returned in build function
/// 
/// ## Examples
/// 
/// example which use all of methods in StrayBuilder
/// 
/// ```
/// let stray = StrayBuilder::new()
///     .with_title("example")
///     .with_size(400, 600)
///     .run_once(init_player_system())
///     .add_system(player_movement_system())
///     .push((Player, Transform2D::ZERO))
///     .insert(10)
///     .add_plugin(MyPlugin)
///     .build();
/// ```
pub struct StrayBuilder{
    render_schedule: systems::Builder,
    global_schedule: systems::Builder,
    once_schedule: systems::Builder,
    stray: Stray,
    settings: Settings,
}

impl StrayBuilder{
    /// Initialize Stray Builder for creating stray app
    pub fn new() -> Self{
        let global_schedule = Schedule::builder();
        let render_schedule = Schedule::builder();
        let once_schedule = Schedule::builder();
        let settings = Settings::default();
        let event_loop = EventLoop::new();
        
        let stray = Stray { 
            global_schedule: None,  
            render_schedule: None,
            global_resources: Resources::default(), 
            render_resources: Resources::default(),
            plugins: vec![], 
            event_loop: event_loop, 
            world: World::default(),
        };

        Self { 
            render_schedule, 
            global_schedule,
            once_schedule,
            stray,
            settings,
        }
    }
    
    /// Sets title it stray settings
    pub fn with_title(mut self, title: &'static str) -> Self{
        self.settings.title = title;
        self
    }

    /// Sets size in stray settings
    pub fn with_size(mut self, width: u32, height: u32) -> Self{
        self.settings.width = width;
        self.settings.height = height;
        self
    }

    /// Adds system to run_once schedule.
    /// Is used for initalizating entites.
    /// Executed while build method is called
    /// 
    /// ### Example
    /// 
    /// ```
    /// Stray::new()
    ///     .run_once()
    ///     .build();
    /// 
    /// #[system]
    /// fn init_player(cmd: &CommandBuffer){
    ///     cmd.push((Player, Transform2D::Zero()));
    /// }
    pub fn run_once<R>(mut self, system: R) -> Self
    where
        R: systems::ParallelRunnable + 'static
    {
        self.once_schedule.add_system(system);
        self
    }

    /// Adds system to global schedule.
    /// Is used for game logic.
    /// Executed in every frame after run method call
    /// 
    /// ### Example
    /// 
    /// ```
    /// Stray::new()
    ///     .add_system(player_movement_system())
    ///     .build()
    ///     .run();
    /// 
    /// #[system(for_each)]
    /// fn init_player(player: &Player, transform: &mut Transform2D){
    ///     transform.position.x += player.velocity.x;
    /// }
    pub fn add_system<S>(mut self, system: S) -> Self
    where
        S: systems::ParallelRunnable + 'static
    {
        self.global_schedule.add_system(system);
        self
    }

    /// Creates entity from given components and pushes it to legion world in stray struct.
    /// Is used for storing data like velocity, hp, etc.
    /// 
    /// ## Example
    /// 
    /// ```
    /// struct Player{
    ///     velocity: Vec2,
    ///     hp: i32,
    /// }
    /// 
    /// struct Ball{
    ///     velocity: Vec2,
    /// }
    /// 
    /// let player = Player{
    ///     velocity: Vec2::new(0.0,0.0),
    ///     hp: 100
    /// };
    /// 
    /// let ball = Ball{
    ///     velocity: Vec2::new(0.0,0.0)
    /// };
    /// Stray::new()
    ///     .push((player,Transform2D::ZERO))
    ///     .push((ball, Transform2D::ZERO));
    /// ```
    pub fn push<T>(mut self, comp: T) ->  Self
    where
        Option<T>: storage::IntoComponentSource
    {
        self.stray.world.push(comp);
        self
    }
    
    /// Inserts resource to global resources
    /// 
    /// ## Example
    /// ```
    /// let number = 100;
    /// let stray = Stray::new()
    ///     .insert(number)
    ///     .build();
    /// 
    /// stray.get_mut_resources().get_mut::<i32>().unwrap() -= 50;
    /// ```
    pub fn insert<T>(mut self, res: T) -> Self
    where
        T: 'static
    {
        self.stray.global_resources.insert(res);
        self
    }

    /// Adds and **builds** plugin 
    /// ## Example
    /// 
    /// ```
    /// struct MyPlugin;
    /// 
    /// impl Plugin for MyPlugin{
    ///     fn build(stray: &mut Stray){
    ///         // code...
    ///     }
    /// }
    /// 
    /// Stray::new()
    ///     .add_plugin(MyPlugin);
    /// ```
    pub fn add_plugin<P>(mut self, plugin: P) -> Self
    where
        P: Plugin + 'static
    {
        plugin.build(&mut self);
        self
    }

    fn init_systems(&mut self){
        self.render_schedule.add_system(read_geometry_system());
        self.render_schedule.add_system(redraw_system());
        self.render_schedule.add_system(read_sprites_system());
    }
    
    /// Finalizes Stray app configuring
    /// It executes run_once schedule and return final stray app. It also adds systems to render schedule for rendering logic
    /// and setup window with stray settings
    pub fn build(mut self) -> Stray{
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(600, 600))
            .build(&self.stray.event_loop).unwrap();
        self.init_systems();
        parse_settings(&self.settings,&window);
        self.stray.global_resources.insert(self.settings);
        self.stray.global_resources.insert(window);
        self.once_schedule.build().execute(&mut self.stray.world, &mut self.stray.global_resources); 
        self.stray.global_schedule = Some(self.global_schedule.build());
        self.stray.render_schedule = Some(self.render_schedule.build());
        self.stray
    }


}


// Other stuff

pub fn resize(render_res: &Resources, global_res: &Resources, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
        global_res.get_mut::<Settings>().unwrap().width = new_size.width;
        global_res.get_mut::<Settings>().unwrap().height = new_size.height;
        render_res.get_mut::<EngineData<SurfaceConfiguration>>().unwrap().0.width = new_size.width;
        render_res.get_mut::<EngineData<SurfaceConfiguration>>().unwrap().0.height = new_size.height;
        render_res.get::<EngineData<Surface>>().unwrap().0.configure(&render_res.get::<EngineData<Device>>().unwrap().0, &render_res.get::<EngineData<SurfaceConfiguration>>().unwrap().0);
    }
}

