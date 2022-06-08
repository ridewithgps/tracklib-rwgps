require_relative 'lib/tracklib-rwgps/version'

Gem::Specification.new do |spec|
  spec.name          = "tracklib-rwgps"
  spec.version       = TracklibRWGPS::VERSION
  spec.authors       = ["Dan Larkin"]
  spec.email         = ["dan@danlarkin.org"]

  spec.summary       = "tracklib-rwgps"
  spec.description   = "RWGPS tracklib-rwgps ruby gem"
  spec.homepage      = "https://ridewithgps.com"
  spec.licenses      = ["Apache-2.0", "MIT"]
  spec.files         = ["tracklib-rwgps.gemspec",
                        "Rakefile",
                        "Gemfile",
                        "lib/tracklib-rwgps.rb",
                        "lib/tracklib-rwgps/version.rb",
                        "Cargo.toml",
                        "Cargo.lock"]
  spec.files        += Dir["src/**/*.rs"]

  spec.required_ruby_version = Gem::Requirement.new(">= 2.3.0")
  spec.require_paths = ["lib"]

  spec.add_development_dependency "rspec"
#  spec.add_development_dependency "tracklib"

  spec.add_dependency 'rutie', '~> 0.0.4'
end
