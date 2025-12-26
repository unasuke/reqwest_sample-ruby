# frozen_string_literal: true

require "mkmf"
require "rb_sys/mkmf"

create_rust_makefile("reqwest_sample/reqwest_sample") do |r|
  r.extra_rustflags = ["--cfg=reqwest_unstable"]
end
