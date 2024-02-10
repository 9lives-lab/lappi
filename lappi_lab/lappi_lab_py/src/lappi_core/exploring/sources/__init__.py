import lappi_core.api_server
from lappi_core.exploring.sources.discogs import discogs_factory
from lappi_core.exploring.sources.chatgpt import chatgpt_factory


source_factories = {
    "discogs.com": discogs_factory,
    "chatgpt": chatgpt_factory
}


def get_exploring_source(source_name):
    return source_factories[source_name]()


def get_artist_description(request):
    source = get_exploring_source(request['source_name'])
    description = source.get_artist_description(request['artist_name'])
    return {
        'description': description
    }


def initialize():
    lappi_core.api_server.add_handler('exploring.sources.get_artist_description', get_artist_description)
