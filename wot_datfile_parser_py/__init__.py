from wot_datfile_parser_py.wot_datfile_parser_py import DatfileParser as _DatfileParser

import json 

class DatfileParser:
    def __init__(self) -> None:
        self.__parser = _DatfileParser()
    
    def parse(self, file) -> dict:
        data = file.read()
        json_string = self.__parser.parse(data)
        return json.loads(json_string)