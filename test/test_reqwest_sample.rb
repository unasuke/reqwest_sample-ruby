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

  def test_client_new_returns_client_instance
    client = ReqwestSample::Client.new
    assert_instance_of ReqwestSample::Client, client
  end

  def test_multiple_clients_can_be_created
    client1 = ReqwestSample::Client.new
    client2 = ReqwestSample::Client.new
    refute_same client1, client2
  end

  def test_client_instance_get_returns_response_body
    client = ReqwestSample::Client.new
    response = client.get("#{TestServerHelper.base_url}/")
    assert_equal "Hello from Puma!", response
  end

  def test_client_instance_get_with_json_endpoint
    client = ReqwestSample::Client.new
    response = client.get("#{TestServerHelper.base_url}/json")
    assert_equal '{"message":"ok"}', response
  end
end
