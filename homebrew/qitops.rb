class Qitops < Formula
  desc "Software Quality Assurance CLI for API, Performance, Security, and Web Testing"
  homepage "https://github.com/qitops/qitops-cli-tools"
  url "https://github.com/qitops/qitops-cli-tools/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "REPLACE_WITH_ACTUAL_SHA256_AFTER_FIRST_RELEASE"
  license "MIT"
  head "https://github.com/qitops/qitops-cli-tools.git", branch: "master"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "QitOps", shell_output("#{bin}/qitops --help")
  end
end
