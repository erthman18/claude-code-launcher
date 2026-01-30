import { useEffect, useState } from 'react';
import { api } from '../api';
import type { DependencyStatus } from '../types';

export const DependencyFrame = () => {
  const [nodejsStatus, setNodejsStatus] = useState<DependencyStatus | null>(null);
  const [claudeStatus, setClaudeStatus] = useState<DependencyStatus | null>(null);
  const [gitbashStatus, setGitbashStatus] = useState<DependencyStatus | null>(null);
  const [nodejsLoading, setNodejsLoading] = useState(false);
  const [claudeLoading, setClaudeLoading] = useState(false);
  const [gitbashLoading, setGitbashLoading] = useState(false);

  // 自动检测(启动时)
  useEffect(() => {
    setTimeout(() => {
      checkInstallationOnly();
    }, 100);
  }, []);

  const checkInstallationOnly = async () => {
    try {
      const nodeResult = await api.checkNodejs();
      await new Promise(resolve => setTimeout(resolve, 200));
      setNodejsStatus(nodeResult);

      const claudeResult = await api.checkClaude();
      await new Promise(resolve => setTimeout(resolve, 200));
      setClaudeStatus(claudeResult);

      const gitbashResult = await api.checkGitbash();
      await new Promise(resolve => setTimeout(resolve, 200));
      setGitbashStatus(gitbashResult);
    } catch (error) {
      console.error('检测失败:', error);
    }
  };

  const checkWithUpdates = async () => {
    setNodejsStatus(null);
    setClaudeStatus(null);
    setGitbashStatus(null);

    try {
      await api.refreshSystemPath();

      const nodeResult = await api.checkNodejsWithUpdate();
      await new Promise(resolve => setTimeout(resolve, 200));
      setNodejsStatus(nodeResult);

      const claudeResult = await api.checkClaudeWithUpdate();
      await new Promise(resolve => setTimeout(resolve, 200));
      setClaudeStatus(claudeResult);

      const gitbashResult = await api.checkGitbashWithUpdate();
      await new Promise(resolve => setTimeout(resolve, 200));
      setGitbashStatus(gitbashResult);
    } catch (error) {
      console.error('检测失败:', error);
    }
  };

  const handleInstallNodejs = async () => {
    setNodejsLoading(true);
    try {
      await api.installNodejs();
      // 安装窗口会自动打开,等待完成后重新检测
      setTimeout(() => {
        checkInstallationOnly();
        setNodejsLoading(false);
      }, 2000);
    } catch (error: any) {
      alert(`安装失败: ${error}`);
      setNodejsLoading(false);
    }
  };

  const handleUpdateNodejs = async () => {
    setNodejsLoading(true);
    try {
      await api.updateNodejs();
      setTimeout(() => {
        checkInstallationOnly();
        setNodejsLoading(false);
      }, 2000);
    } catch (error: any) {
      alert(`更新失败: ${error}`);
      setNodejsLoading(false);
    }
  };

  const handleInstallClaude = async () => {
    setClaudeLoading(true);
    try {
      await api.installClaude();
      setTimeout(() => {
        checkInstallationOnly();
        setClaudeLoading(false);
      }, 2000);
    } catch (error: any) {
      alert(`安装失败: ${error}`);
      setClaudeLoading(false);
    }
  };

  const handleUpdateClaude = async () => {
    setClaudeLoading(true);
    try {
      await api.updateClaude();
      setTimeout(() => {
        checkInstallationOnly();
        setClaudeLoading(false);
      }, 2000);
    } catch (error: any) {
      alert(`更新失败: ${error}`);
      setClaudeLoading(false);
    }
  };

  const handleInstallGitbash = async () => {
    setGitbashLoading(true);
    try {
      await api.installGitbash();
      setTimeout(() => {
        checkInstallationOnly();
        setGitbashLoading(false);
      }, 2000);
    } catch (error: any) {
      alert(`安装失败: ${error}`);
      setGitbashLoading(false);
    }
  };

  const handleUpdateGitbash = async () => {
    setGitbashLoading(true);
    try {
      await api.updateGitbash();
      setTimeout(() => {
        checkInstallationOnly();
        setGitbashLoading(false);
      }, 2000);
    } catch (error: any) {
      alert(`更新失败: ${error}`);
      setGitbashLoading(false);
    }
  };

  const renderStatus = (status: DependencyStatus | null) => {
    if (!status) {
      return <span className="text-[#999999] text-[10px]">⏳</span>;
    }

    if (!status.installed) {
      return <span className="text-error text-[10px]">✗</span>;
    }

    if (status.update_available && status.latest_version) {
      return (
        <span className="text-warning text-[10px]">
          ⚠ {status.version} → {status.latest_version}
        </span>
      );
    }

    return <span className="text-success text-[10px]">✓ {status.version}</span>;
  };

  const renderNodejsButton = () => {
    if (!nodejsStatus || nodejsLoading) {
      return null;
    }

    if (!nodejsStatus.installed) {
      return (
        <button
          onClick={handleInstallNodejs}
          disabled={nodejsLoading}
          className="px-3 py-1 text-[10px] bg-primary hover:bg-primary-hover rounded text-white"
        >
          {nodejsLoading ? '安装中...' : '安装'}
        </button>
      );
    }

    if (nodejsStatus.update_available) {
      return (
        <button
          onClick={handleUpdateNodejs}
          disabled={nodejsLoading}
          className="px-3 py-1 text-[10px] bg-primary hover:bg-primary-hover rounded text-white"
        >
          {nodejsLoading ? '更新中...' : '更新'}
        </button>
      );
    }

    return null;
  };

  const renderClaudeButton = () => {
    if (!claudeStatus || claudeLoading) {
      return null;
    }

    if (!claudeStatus.installed) {
      return (
        <button
          onClick={handleInstallClaude}
          disabled={claudeLoading}
          className="px-3 py-1 text-[10px] bg-primary hover:bg-primary-hover rounded text-white"
        >
          {claudeLoading ? '安装中...' : '安装'}
        </button>
      );
    }

    if (claudeStatus.update_available) {
      return (
        <button
          onClick={handleUpdateClaude}
          disabled={claudeLoading}
          className="px-3 py-1 text-[10px] bg-primary hover:bg-primary-hover rounded text-white"
        >
          {claudeLoading ? '更新中...' : '更新'}
        </button>
      );
    }

    return null;
  };

  const renderGitbashButton = () => {
    if (!gitbashStatus || gitbashLoading) {
      return null;
    }

    if (!gitbashStatus.installed) {
      return (
        <button
          onClick={handleInstallGitbash}
          disabled={gitbashLoading}
          className="px-3 py-1 text-[10px] bg-primary hover:bg-primary-hover rounded text-white"
        >
          {gitbashLoading ? '安装中...' : '安装'}
        </button>
      );
    }

    if (gitbashStatus.update_available) {
      return (
        <button
          onClick={handleUpdateGitbash}
          disabled={gitbashLoading}
          className="px-3 py-1 text-[10px] bg-primary hover:bg-primary-hover rounded text-white"
        >
          {gitbashLoading ? '更新中...' : '更新'}
        </button>
      );
    }

    return null;
  };

  return (
    <div className="px-5 py-2">
      <div className="card-frame">
        <div className="flex items-center gap-4">
          <h2 className="text-base font-bold">系统依赖(先装node)</h2>

          <div className="flex items-center gap-2">
            <span className="text-[10px]">Node.js:</span>
            {renderStatus(nodejsStatus)}
            {renderNodejsButton()}
          </div>

          <div className="flex items-center gap-2">
            <span className="text-[10px]">Git:</span>
            {renderStatus(gitbashStatus)}
            {renderGitbashButton()}
          </div>

          <div className="flex items-center gap-2">
            <span className="text-[10px]">Claude Code:</span>
            {renderStatus(claudeStatus)}
            {renderClaudeButton()}
          </div>

          <div className="flex-1" />

          <button
            onClick={checkWithUpdates}
            className="px-4 py-1 text-[10px] bg-[#666666] hover:bg-[#555555] text-white rounded"
          >
            检查更新
          </button>
        </div>
      </div>
    </div>
  );
};
