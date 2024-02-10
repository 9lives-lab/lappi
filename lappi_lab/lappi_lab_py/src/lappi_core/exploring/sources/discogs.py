import lappi_core.registry
import discogs_client
from lappi_core.exploring.sources.api import ExploringSource
from lappi_core.api_server import ApiException


class DiscogsExploringSource(ExploringSource):
    def __init__(self):
        self.user_token = lappi_core.registry.get_value('exploring.sources.discogs.user_token')
        self.client = discogs_client.Client('ExampleDiscogsExplorer/1.0', user_token=self.user_token)

    def get_capabilities(self):
        return None

    def get_artist_description(self, artist_name):
        try:
            # Search for the artist on Discogs
            results = self.client.search(artist_name, type='artist')

            if results and results[0].name.lower() == artist_name.lower():
                artist = results[0]
                description = artist.profile
                return description
            else:
                raise Exception(f"Artist '{artist_name}' not found on Discogs.")

        except discogs_client.exceptions.HTTPError:
            raise ApiException("HTTP error")


def discogs_factory():
    source = DiscogsExploringSource()
    return source
