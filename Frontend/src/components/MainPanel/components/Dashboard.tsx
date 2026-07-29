import React, { useState } from 'react'
import type { Stage } from '../../../types'
import ExecutionMatrix from './dashboard/ExecutionMatrix'
import BudgetDonutChart from './dashboard/BudgetDonutChart'
import styles from '../MainPanel.module.scss'

interface DashboardProps {
  projectId: string | null
  stagesLoading: boolean
  stages: Stage[]
  sortedStagesForDashboard: Stage[]
  getStageLabel: (stage: Stage) => string
  dispatch: any
}

export default function Dashboard({
  projectId,
  stagesLoading,
  stages,
  sortedStagesForDashboard,
  getStageLabel,
  dispatch,
}: DashboardProps) {
  const [matrixOpen, setMatrixOpen] = useState(true)
  const [budgetOpen, setBudgetOpen] = useState(true)

  if (!projectId || stagesLoading || stages.length === 0) {
    return null
  }

  return (
    <div className={styles.tabContentDashboard}>
      {/* Panel 1: Checkpoint Matrix */}
      <div className={styles.dashboardContainer}>
        <div className={styles.dashboardHeader} onClick={() => setMatrixOpen((o) => !o)}>
          <span className={styles.dashboardHeaderTitle}>Матрица выполнения</span>
          <span className={styles.dashboardHeaderToggle}>
            {matrixOpen ? 'Свернуть ▲' : 'Развернуть ▼'}
          </span>
        </div>
        {matrixOpen && (
          <div className={styles.dashboardBody}>
            <ExecutionMatrix
              sortedStagesForDashboard={sortedStagesForDashboard}
              getStageLabel={getStageLabel}
              dispatch={dispatch}
            />
          </div>
        )}
      </div>

      {/* Panel 2: Budget Distribution */}
      <div className={styles.dashboardContainer}>
        <div className={styles.dashboardHeader} onClick={() => setBudgetOpen((o) => !o)}>
          <span className={styles.dashboardHeaderTitle}>Распределение бюджета</span>
          <span className={styles.dashboardHeaderToggle}>
            {budgetOpen ? 'Свернуть ▲' : 'Развернуть ▼'}
          </span>
        </div>
        {budgetOpen && (
          <div className={styles.dashboardBody}>
            <BudgetDonutChart
              stages={stages}
              sortedStagesForDashboard={sortedStagesForDashboard}
              getStageLabel={getStageLabel}
              dispatch={dispatch}
            />
          </div>
        )}
      </div>
    </div>
  )
}
