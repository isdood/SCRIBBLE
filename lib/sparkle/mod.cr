module Scribble
  module Sparkle
    VERSION = "1.0.0"
    CREATED = "2025-01-25 03:09:43"
    TENDER  = "isdood"

    def self.root_path
      File.dirname(__FILE__)
    end

    def self.pattern_path
      File.join(root_path, "patterns")
    end

    def self.config_path
      File.join(root_path, "config.sparkle")
    end
  end
end
