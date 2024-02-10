import lappi_core.registry
import openai
from lappi_core.exploring.sources.api import ExploringSource


class ChatgptExploringSource(ExploringSource):
    def __init__(self):
        self.user_token = lappi_core.registry.get_value('exploring.sources.chatgpt.user_token')

    def get_capabilities(self):
        return None

    def get_artist_description(self, artist_name):
        openai.api_key = self.user_token

        prompt = 'Hi, could you please tell me a history of music artist "' + artist_name + '"?'
        messages = [
            {"role": "system", "content": "You are a intelligent assistant."},
            {"role": "user", "content": prompt}
        ]
        response = openai.ChatCompletion.create(
            model="gpt-3.5-turbo",
            messages=messages,
            temperature=0,
        )
        return response.choices[0].message["content"]


def chatgpt_factory():
    source = ChatgptExploringSource()
    return source
