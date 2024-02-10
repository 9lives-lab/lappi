import lappi_core.api_server
import lappi_core.registry
import lappi_core.exploring.sources


def initialize():
    lappi_core.api_server.initialize()
    lappi_core.registry.initialize()
    lappi_core.exploring.sources.initialize()
