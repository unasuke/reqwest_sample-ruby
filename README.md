# ReqwestSample

TODO: Delete this and the text below, and describe your gem

Welcome to your new gem! In this directory, you'll find the files you need to be able to package up your Ruby library into a gem. Put your Ruby code in the file `lib/reqwest_sample`. To experiment with that code, run `bin/console` for an interactive prompt.

## Installation

TODO: Replace `UPDATE_WITH_YOUR_GEM_NAME_IMMEDIATELY_AFTER_RELEASE_TO_RUBYGEMS_ORG` with your gem name right after releasing it to RubyGems.org. Please do not do it earlier due to security reasons. Alternatively, replace this section with instructions to install your gem from git if you don't plan to release to RubyGems.org.

Install the gem and add to the application's Gemfile by executing:

```bash
bundle add UPDATE_WITH_YOUR_GEM_NAME_IMMEDIATELY_AFTER_RELEASE_TO_RUBYGEMS_ORG
```

If bundler is not being used to manage dependencies, install the gem by executing:

```bash
gem install UPDATE_WITH_YOUR_GEM_NAME_IMMEDIATELY_AFTER_RELEASE_TO_RUBYGEMS_ORG
```

## Usage

```ruby
require "reqwest_sample"

# Create a client
client = ReqwestSample::Client.new

# Send a GET request
response = client.get("https://example.com")

# Access response data
response.status   # => 200
response.version  # => "HTTP/3.0" or "HTTP/2.0" or "HTTP/1.1"
response.headers  # => {"content-type" => "text/html", ...}
response.body     # => "<!DOCTYPE html>..."

# Inspect the response
response.inspect
# => #<ReqwestSample::Response status=200 version="HTTP/2.0" headers={9 entries} body="<!DOCTYPE html>...">
```

### HTTP/3 Support

This gem automatically uses HTTP/3 when the target server supports it. The protocol is determined by looking up the HTTPS DNS record (ALPN) before sending the request. If HTTP/3 fails, it falls back to HTTP/2 or HTTP/1.1.

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake test` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and the created tag, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/unasuke/reqwest_sample. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [code of conduct](https://github.com/unasuke/reqwest_sample/blob/main/CODE_OF_CONDUCT.md).

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).

## Code of Conduct

Everyone interacting in the ReqwestSample project's codebases, issue trackers, chat rooms and mailing lists is expected to follow the [code of conduct](https://github.com/unasuke/reqwest_sample/blob/main/CODE_OF_CONDUCT.md).
