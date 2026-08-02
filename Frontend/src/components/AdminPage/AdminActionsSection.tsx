import React, { useState } from 'react'
import { useTriggerDigestMutation } from '../../store/api/adminApi'
import styles from './AdminPage.module.scss'

export default function AdminActionsSection() {
  const [triggerDigest, { isLoading }] = useTriggerDigestMutation()
  const [successMsg, setSuccessMsg] = useState<string | null>(null)

  const handleSendDigest = async () => {
    setSuccessMsg(null)
    const res = await triggerDigest()
    if (!('error' in res)) {
      setSuccessMsg('Дайджест дедлайнов успешно отправлен!')
      setTimeout(() => setSuccessMsg(null), 4000)
    }
  }

  return (
    <div className={styles.section}>
      <div className={styles.tableContainer} style={{ padding: 24 }}>
        <h3 className={styles.modalTitle}>Системные операции</h3>
        <p className={styles.statLabel} style={{ marginBottom: 20 }}>
          Выполнение оперативных административных команд и отсылки уведомлений.
        </p>

        <div style={{ display: 'flex', flexDirection: 'column', gap: 16, maxWidth: 480 }}>
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'space-between',
              padding: 16,
              borderRadius: 12,
              background: 'rgba(255, 255, 255, 0.03)',
              border: '1px solid rgba(255, 255, 255, 0.08)',
            }}
          >
            <div>
              <strong style={{ display: 'block', fontSize: '0.95rem' }}>
                Рассылка дайджеста дедлайнов
              </strong>
              <span className={styles.statLabel}>
                Отправить электронное письмо о горящих этапах ответственным ГИПам
              </span>
            </div>
            <button
              className={styles.actionBtn}
              onClick={handleSendDigest}
              disabled={isLoading}
            >
              {isLoading ? 'Отправка…' : 'Запустить'}
            </button>
          </div>
          {successMsg && (
            <div
              style={{
                color: '#34d399',
                fontSize: '0.85rem',
                fontWeight: 600,
                padding: '8px 12px',
                borderRadius: 8,
                background: 'rgba(16, 185, 129, 0.1)',
              }}
            >
              {successMsg}
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
