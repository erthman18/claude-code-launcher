import { useSortable } from '@dnd-kit/sortable';
import { CSS } from '@dnd-kit/utilities';
import { ProjectCard } from './ProjectCard';
import type { Project } from '../types/project';

interface SortableProjectCardProps {
  project: Project;
  platform: string;
  onLaunch: (id: string) => void;
  onEdit: (id: string) => void;
}

export const SortableProjectCard: React.FC<SortableProjectCardProps> = ({
  project,
  platform,
  onLaunch,
  onEdit,
}) => {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id: project.id });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : 1,
  };

  return (
    <div ref={setNodeRef} style={style} {...attributes} {...listeners}>
      <ProjectCard
        project={project}
        platform={platform}
        onLaunch={onLaunch}
        onEdit={onEdit}
        isDragging={isDragging}
      />
    </div>
  );
};
