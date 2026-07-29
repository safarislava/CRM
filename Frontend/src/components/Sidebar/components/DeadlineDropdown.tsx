import React from 'react'
import type { StageWithProjectTitle } from '../../../types'
import { selectProject, selectStage } from '../../../store/uiSlice'
import styles from '../Sidebar.module.scss'

function deadlineDiffDays(iso: string) {
  const d = new Date(iso)
  const deadline = new Date(d.getFullYear(), d.getMonth(), d.getDate())
  const now = new Date()
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate())
  return Math.round((deadline.getTime() - today.getTime()) / 86_400_000)
}

function formatDeadlineDate(iso: string): string {
  const d = new Date(iso)
  const diff = deadlineDiffDays(iso)
  if (diff < 0) return d.toLocaleDateString('ru-RU', { day: 'numeric', month: 'short' })
  if (diff === 0) return 'Сегодня'
  if (diff === 1) return 'Завтра'
  return d.toLocaleDateString('ru-RU', { day: 'numeric', month: 'short' })
}

function deadlineUrgency(iso: string): 'overdue' | 'urgent' | 'soon' | 'normal' {
  const diff = deadlineDiffDays(iso)
  if (diff < 0) return 'overdue'
  if (diff <= 1) return 'urgent'
  if (diff <= 7) return 'soon'
  return 'normal'
}

interface DeadlineDropdownProps {
  dropdownRef: React.RefObject<HTMLDivElement>
  deadlineItems: StageWithProjectTitle[]
  onClose: () => void
  dispatch: any
}

export default function DeadlineDropdown({
  dropdownRef,
  deadlineItems,
  onClose,
  dispatch,
}: DeadlineDropdownProps) {
  return (
    <div ref={dropdownRef} className={styles.deadlineDropdown}>
      <div className={styles.deadlineDropdownHeader}>Ближайшие дедлайны</div>
      {deadlineItems.length === 0 ? (
        <div className={styles.deadlineEmpty}>Нет предстоящих дедлайнов</div>
      ) : (
        deadlineItems.map((item) => (
          <button
            key={`${item.stage.project_id}-${item.stage.position}`}
            className={styles.deadlineItem}
            onClick={() => {
              dispatch(selectProject(item.stage.project_id))
              dispatch(
                selectStage({
                  parentPosition: item.stage.parent_position,
                  position: item.stage.position,
                }),
              )
              onClose()
            }}
          >
            <span
              className={`${styles.deadlineDate} ${
                styles[`deadline_${deadlineUrgency(item.stage.deadline!)}`]
              }`}
            >
              {formatDeadlineDate(item.stage.deadline!)}
            </span>
            <div className={styles.deadlineInfo}>
              <span className={styles.deadlineProject}>{item.project_title}</span>
              <span className={styles.deadlineStage}>{item.stage.title}</span>
            </div>
          </button>
        ))
      )}
    </div>
  )
}

export { deadlineDiffDays }
