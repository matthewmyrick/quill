class QuillTask < Formula
  desc "Git-context-aware task management TUI with local and MongoDB storage support"
  homepage "https://github.com/MatthewMyrick/quill"
  url "https://github.com/MatthewMyrick/quill/releases/download/v0.1.0/quill-task-x86_64-apple-darwin.tar.gz"
  sha256 "PLACEHOLDER_SHA256" # This will be updated by the GitHub Action
  license "MIT"
  version "0.1.0"

  depends_on "rust" => :build

  def install
    bin.install "quill-task" => "quill"
  end

  test do
    # Test that the binary exists and runs
    assert_match "Error: This application requires a proper terminal to run", shell_output("#{bin}/quill 2>&1", 1)
  end
end