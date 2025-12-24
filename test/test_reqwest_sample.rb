# frozen_string_literal: true

require "test_helper"

class TestReqwestSample < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::ReqwestSample::VERSION
  end

  def test_client_get_returns_response_body
    response = ReqwestSample::Client.get("#{TestServerHelper.base_url}/")
    assert_equal "Hello from Puma!", response
  end

  def test_client_get_with_json_endpoint
    response = ReqwestSample::Client.get("#{TestServerHelper.base_url}/json")
    assert_equal '{"message":"ok"}', response
  end

  def test_client_get_with_connection_refused_raises_error
    assert_raises(RuntimeError) do
      ReqwestSample::Client.get("http://example.invalid/")
    end
  end
end
