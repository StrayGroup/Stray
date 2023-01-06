import ctypes


class entity_native(ctypes.Structure):
    _fields_ = [("components_ids", ctypes.c_char_p)]

# Initialize Classes

class settings_native(ctypes.Structure):
    _fields_ = [("title",ctypes.c_char_p)]

class stray_native(ctypes.Structure):
    _fields_ = [("settings",settings_native)]

    

class stray:
    def __init__(self, title = "Stray App"):
        stray_be = ctypes.WinDLL("./python/api/stray_be.dll")

        stray_be.create_settings.restype = settings_native
        stray_be.create_settings.argtypes = [ctypes.c_char_p]

        stray_be.create_stray.restype = stray_native
        stray_be.create_stray.argtypes = [settings_native]

        settings = stray_be.create_settings(title.encode())
        stray =  stray_be.create_stray(settings)

        stray_be.run_stray(stray)
        print("cos")



        