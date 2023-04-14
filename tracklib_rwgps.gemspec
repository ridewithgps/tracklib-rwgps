require_relative 'lib/tracklib_rwgps/version'

Gem::Specification.new do |spec|
  spec.name          = "tracklib_rwgps"
  spec.version       = TracklibRWGPS::VERSION
  spec.authors       = ["Dan Larkin"]
  spec.email         = ["dan@danlarkin.org"]

  spec.summary       = "tracklib_rwgps"
  spec.description   = "RWGPS tracklib_rwgps ruby gem"
  spec.homepage      = "https://ridewithgps.com"
  spec.licenses      = ["Apache-2.0", "MIT"]
  spec.files         = ["tracklib_rwgps.gemspec",
                        "Rakefile",
                        "Gemfile",
                        "lib/tracklib_rwgps.rb",
                        "lib/tracklib_rwgps/version.rb",
                        "Cargo.toml",
                        "Cargo.lock"]
  spec.files        += Dir["src/**/*.rs"]

  spec.required_ruby_version = Gem::Requirement.new(">= 2.3.0")
  spec.require_paths = ["lib"]

  spec.add_development_dependency "rspec"
#  spec.add_development_dependency "tracklib"

  spec.add_dependency 'rutie', '~> 0.0.4'
end
