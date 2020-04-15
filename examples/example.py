# This is an example file to show what scripting can do

from star_engine import System, Read, Write


class SomeSystem(System):
    @staticmethod
    def data():
        return [Read("something"), Write("something")]


systems = [SomeSystem()]
