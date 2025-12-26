# frozen_string_literal: true

require_relative "reqwest_sample/version"
require_relative "reqwest_sample/reqwest_sample"

module ReqwestSample
  class Error < StandardError; end

  class Response
    def inspect
      body_preview = (body.length > 50) ? "#{body[0, 50]}..." : body
      "#<#{self.class} status=#{status} version=#{version.inspect} headers={#{headers.size} entries} body=#{body_preview.inspect}>"
    end
  end
end
