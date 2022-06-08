require 'rspec/core/rake_task'

desc 'Build Rust extension'
task :build_lib do
  sh 'cargo build --release'
end

desc 'bundle install'
task :bundle_install do
  sh 'bundle install'
end

RSpec::Core::RakeTask.new(spec: [:bundle_install, :build_lib]) do |t|
  t.pattern = "spec/**/*_spec.rb"
end

task :default => :spec
task :test => :spec
