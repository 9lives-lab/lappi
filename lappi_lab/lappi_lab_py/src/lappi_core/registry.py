import lappi_core.api_server

registry_values = {}


def set_value(key, value):
    registry_values[key] = value


def get_value(key):
    return registry_values[key]


def set_values_request(request):
    values = request['registry_values']
    for key in values:
        set_value(key, values[key])


def initialize():
    lappi_core.api_server.add_handler('registry.set_values', set_values_request)
