class Work < Formula
  desc "Interactive git worktree manager"
  homepage "https://github.com/qiangfoo/homebrew-tap"
  version "0.2.4"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/qiangfoo/homebrew-tap/releases/download/v#{version}/work-aarch64-apple-darwin.tar.gz"
      sha256 "fe3c6c214a0549b9d6bd64a4a679a323b11641235b7cc6327242369ffa505089"
    end
    on_intel do
      url "https://github.com/qiangfoo/homebrew-tap/releases/download/v#{version}/work-x86_64-apple-darwin.tar.gz"
      sha256 "5f50f23df0cbba3f8a52b2af371e2ca871cbce8e924d4c2affc589befbd849e8"
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
