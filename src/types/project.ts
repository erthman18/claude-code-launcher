export interface ProjectConfig {
  mode: 'claude' | 'custom';
  proxy: string;
  model: string;
  base_url: string;
  token: string;
  skip_permissions: boolean;
}

export interface Project {
  id: string;
  name: string;
  working_directory: string;
  config: ProjectConfig;
  is_default: boolean;
  created_at: number;
  updated_at: number;
  last_launched_at?: number;
  is_pinned: boolean;
  pinned_at?: number;
  sort_order: number;
}

export interface CreateProjectInput {
  name: string;
  working_directory: string;
  config: ProjectConfig;
}

export interface UpdateProjectInput {
  name?: string;
  working_directory?: string;
  config?: ProjectConfig;
  is_pinned?: boolean;
}

export interface ProjectOrderItem {
  id: string;
  sort_order: number;
}

export interface PinnedOrderItem {
  id: string;
  pinned_at: number;
}

export const DEFAULT_PROJECT_CONFIG: ProjectConfig = {
  mode: 'claude',
  proxy: '',
  model: 'qwen3-coder-480b-a35b',
  base_url: 'http://litellm.uattest.weoa.com',
  token: '',
  skip_permissions: true,
};
