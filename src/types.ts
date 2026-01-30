export interface DependencyStatus {
  installed: boolean;
  version: string | null;
  meets_requirement: boolean;
  latest_version: string | null;
  update_available: boolean;
  error: string | null;
}

export interface AppConfig {
  mode: 'claude' | 'custom';
  proxy: string;
  model: string;
  base_url: string;
  token: string;
  skip_permissions: boolean;
}

export const DEFAULT_CONFIG: AppConfig = {
  mode: 'claude',
  proxy: '',
  model: 'qwen3-coder-480b-a35b',
  base_url: 'http://litellm.uattest.weoa.com',
  token: '',
  skip_permissions: true,
};

export const MODEL_OPTIONS = [
  'deepseek-v3',
  'qwen3-235b-a22b',
  'qwen3-coder-480b-a35b',
];
