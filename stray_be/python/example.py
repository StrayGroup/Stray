from esper import *
from dataclasses import dataclass as component
from api.stray import stray


@component
class Player:
    position: list

class Movement(Processor):
    def process(self):
        print("hello")

world = World()
player = world.create_entity((Player(position=[10,20])))
world.add_processor(Movement())
stray("appka")
