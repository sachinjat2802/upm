# CPM Ruby on Rails Integration Helper
#
# Usage in config/application.rb:
#     require_relative '../sdk/ruby/cpm_rails'
#     config.middleware.use CpmRails::Middleware

module CpmRails
  class Middleware
    def initialize(app)
      @app = app
    end

    def call(env)
      env['cpm.bridge'] = true
      @app.call(env)
    end
  end
end
