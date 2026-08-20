# CPM Ruby Host Process — stdio RPC bridge (upm-bridge/1)
# Implements 4-byte Big-Endian length-prefixed JSON framing over stdio.

require 'json'
require 'digest'

$stdout.sync = true

def main
  loop do
    header = $stdin.read(4)
    break if header.nil? || header.bytesize < 4

    length = header.unpack1('N')
    break if length > 64 * 1024 * 1024

    payload = $stdin.read(length)
    break if payload.nil? || payload.bytesize < length

    env = JSON.parse(payload)
    if env['type'] == 'request'
      resp = handle_request(env)
      send_response(resp)
    elsif env['type'] == 'ping'
      send_response({ 'type' => 'pong', 'id' => env['id'] })
    end
  rescue EOFError, IOError
    break
  rescue => e
    $stderr.puts "Ruby host error: #{e.message}"
  end
end

def handle_request(req)
  resp = { 'type' => 'response', 'id' => req['id'] }
  method_name = req['method']
  args = req['args'] || []

  case method_name
  when 'ping'
    resp['result'] = 'pong'
  when 'echo'
    resp['result'] = args[0]
  when 'math.sqrt'
    val = args[0].to_f
    resp['result'] = Math.sqrt(val)
  when 'digest.sha256'
    val = args[0].to_s
    resp['result'] = Digest::SHA256.hexdigest(val)
  when '__inspect__'
    resp['result'] = [
      { 'name' => 'math.sqrt', 'description' => 'Ruby Math.sqrt square root calculation' },
      { 'name' => 'digest.sha256', 'description' => 'Ruby Digest::SHA256 digest calculation' },
      { 'name' => 'echo', 'description' => 'Echo input value' },
      { 'name' => 'ping', 'description' => 'Keepalive ping/pong' }
    ]
  else
    resp['error'] = { 'error_type' => 'MethodNotFoundError', 'message' => "Method '#{method_name}' not registered on Ruby host" }
  end

  resp
end

def send_response(env)
  json_bytes = env.to_json.b
  header = [json_bytes.bytesize].pack('N')
  $stdout.write(header)
  $stdout.write(json_bytes)
  $stdout.flush
end

main if __FILE__ == $0
