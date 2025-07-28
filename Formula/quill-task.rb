class QuillTask < Formula
  desc "Git-context-aware task management TUI with local and MongoDB storage support"
  homepage "https://github.com/MatthewMyrick/quill"
  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/MatthewMyrick/quill/releases/download/v0.1.0/quill-task-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_INTEL"
    else
      url "https://github.com/MatthewMyrick/quill/releases/download/v0.1.0/quill-task-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_ARM"
    end
  end
  license "MIT"
  version "0.1.0"

  depends_on "rust" => :build

  def install
    if Hardware::CPU.intel?
      bin.install "quill-task-x86_64-apple-darwin" => "quill"
    else
      bin.install "quill-task-aarch64-apple-darwin" => "quill"
    end
  end

  test do
    # Test that the binary exists and runs
    assert_match "Error: This application requires a proper terminal to run", shell_output("#{bin}/quill 2>&1", 1)
  end
end