import abc


class ExploringSource(abc.ABC):
    @abc.abstractmethod
    def get_capabilities(self):
        pass

    @abc.abstractmethod
    def get_artist_description(self, artist_name):
        pass
