# frozen_string_literal: true

$LOAD_PATH.unshift File.expand_path("../lib", __dir__)
require "reqwest_sample"

require "minitest/autorun"

module TestServerHelper
  TEST_PORT = 19292
  TEST_HOST = "127.0.0.1"

  class << self
    attr_accessor :server_pid

    def start_server
      return if @server_pid

      @server_pid = fork do
        require "puma"

        app = ->(env) {
          case env["PATH_INFO"]
          when "/"
            [200, {"content-type" => "text/plain"}, ["Hello from Puma!"]]
          when "/json"
            [200, {"content-type" => "application/json"}, ['{"message":"ok"}']]
          else
            [404, {"content-type" => "text/plain"}, ["Not Found"]]
          end
        }

        server = Puma::Server.new(app)
        server.add_tcp_listener(TEST_HOST, TEST_PORT)
        server.run
        sleep
      end

      # Wait for server to be ready
      sleep 0.5
    end

    def stop_server
      return unless @server_pid

      Process.kill("TERM", @server_pid)
      Process.wait(@server_pid)
      @server_pid = nil
    end

    def base_url
      "http://#{TEST_HOST}:#{TEST_PORT}"
    end
  end
end

TestServerHelper.start_server

Minitest.after_run do
  TestServerHelper.stop_server
end
