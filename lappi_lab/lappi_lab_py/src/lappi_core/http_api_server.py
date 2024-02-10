from flask import Flask, jsonify, request
import lappi_core.api_server

app = Flask("Lappi API")


@app.route('/make_request', methods=['POST'])
def request_handler():
    if request.method == 'POST':
        args = request.args
        key = args.get('key')
        request_data = request.get_json()
        resp_data = lappi_core.api_server.handle_request(key, request_data)

        if resp_data is None:
            resp_data = {
                'empty_response': True
            }

        return {
            'data': resp_data
        }


def run():
    app.run(host='0.0.0.0', port=5000)

