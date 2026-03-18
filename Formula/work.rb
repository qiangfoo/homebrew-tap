class Work < Formula
  desc "Interactive git worktree manager"
  homepage "https://github.com/qiangfoo/homebrew-work"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/qiangfoo/homebrew-work/releases/download/v#{version}/work-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER"
    end
    on_intel do
      url "https://github.com/qiangfoo/homebrew-work/releases/download/v#{version}/work-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER"
    end
  end

  def install
    bin.install "work"
  end

  def caveats
    <<~EOS
      Add the following to your ~/.zshrc:
        eval "$(work init)"
    EOS
  end
end
