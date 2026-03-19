class Work < Formula
  desc "Interactive git worktree manager"
  homepage "https://github.com/qiangfoo/homebrew-tap"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/qiangfoo/homebrew-tap/releases/download/v#{version}/work-aarch64-apple-darwin.tar.gz"
      sha256 "decfc090b503e2bb07d368ca204a60c3e6292455431a2ac25c10ea481e2ab1fa"
    end
    on_intel do
      url "https://github.com/qiangfoo/homebrew-tap/releases/download/v#{version}/work-x86_64-apple-darwin.tar.gz"
      sha256 "a2a4085db984b5da01024c9bec02a7f8c61bde042b0ca46c2fa3b76d883d52c0"
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
