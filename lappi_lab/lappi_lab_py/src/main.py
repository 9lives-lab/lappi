import sys

import scenarios.server.run_flask_server

scenariosDict = {
    "server.run_flask_server":      scenarios.server.run_flask_server.run,
}

if __name__ == '__main__':
    script_name = sys.argv[1]
    print(f'Script: {script_name}')

    scenariosDict[script_name]()

