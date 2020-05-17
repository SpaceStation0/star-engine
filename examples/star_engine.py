class Read:
    """
    An accessor to read a global resource
    """
    def __init__(self, class_name):
        self.class_name = class_name


class Write:
    """
    An accessor to read/write a global resource
    """
    def __init__(self, class_name):
        self.class_name = class_name


class Entities:
    """
    An accessor to read all entities in the world
    """
    def __init__(self):
        pass
    '''
    Returns a list of component values with a certain value.
    '''
    def filter(self, *args):
        pass


class System:
    """
    A system is a class which modifies a certain set of components each tick of the application
    """
    @staticmethod
    def data():
        pass

    def run(self, data):
        pass
