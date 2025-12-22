# frozen_string_literal: true

require "bundler/gem_tasks"
require "minitest/test_task"

Minitest::TestTask.create

require "standard/rake"

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("reqwest_sample.gemspec")

RbSys::ExtensionTask.new("reqwest_sample", GEMSPEC) do |ext|
  ext.lib_dir = "lib/reqwest_sample"
end

task default: %i[compile test standard]
