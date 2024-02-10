import os
from pathlib import Path


def get_dataset_path(name: str):
    path = Path(os.getcwd())
    return path.parent.joinpath("lappi_dataset").joinpath(name)
