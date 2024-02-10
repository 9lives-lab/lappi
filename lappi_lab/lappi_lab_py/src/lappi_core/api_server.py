class ApiException(Exception):
    def __init__(self, message):
        self.message = message
        super().__init__(self.message)


handlers = {}


def add_handler(key, handler_func):
    handlers[key] = handler_func


def handle_request(key, req):
    handler_func = handlers[key]
    return handler_func(req)


def check_connection(request):
    return request


def initialize():
    add_handler('check_connection', check_connection)

