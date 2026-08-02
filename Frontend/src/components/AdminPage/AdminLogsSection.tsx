import React, { useState } from 'react'
import { useGetAdminLogsQuery, type LogEntry } from '../../store/api/adminApi'
import styles from './AdminPage.module.scss'

export default function AdminLogsSection() {
  const [levelFilter, setLevelFilter] = useState<'ALL' | 'INFO' | 'WARN' | 'ERROR'>('ALL')
  const [query, setQuery] = useState('')
  const [selectedLog, setSelectedLog] = useState<LogEntry | null>(null)

  const { data, isLoading, refetch } = useGetAdminLogsQuery({
    level: levelFilter,
    query: query || undefined,
    limit: 200,
  })

  const logs = data?.logs ?? []
  const files = data?.files ?? []

  return (
    <div className={styles.section}>
      <div className={styles.tableContainer}>
        <div className={styles.toolbar}>
          <div className={styles.searchBox}>
            <SearchIcon />
            <input
              placeholder="Поиск по системным логам…"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
            />
          </div>

          <div style={{ display: 'flex', gap: 8, flexWrap: 'wrap' }}>
            {(['ALL', 'INFO', 'WARN', 'ERROR'] as const).map((lvl) => (
              <button
                key={lvl}
                className={`${styles.tabBtn} ${levelFilter === lvl ? styles.tabActive : ''}`}
                style={{ padding: '6px 12px', fontSize: '0.8rem' }}
                onClick={() => setLevelFilter(lvl)}
              >
                {lvl === 'ALL' ? 'Все уровни' : lvl}
              </button>
            ))}
            <button
              className={styles.iconBtn}
              onClick={() => refetch()}
              title="Обновить логи"
            >
              <RefreshIcon />
            </button>
          </div>
        </div>

        {isLoading ? (
          <div style={{ padding: 24, textAlign: 'center' }}>Загрузка логов…</div>
        ) : logs.length === 0 ? (
          <div style={{ padding: 32, textAlign: 'center', color: '#94a3b8' }}>
            Логи не найдены
          </div>
        ) : (
          <div className={styles.logConsole}>
            {logs.map((log, index) => (
              <div
                key={index}
                className={styles.logRow}
                onClick={() => setSelectedLog(log)}
                style={{ cursor: 'pointer' }}
              >
                {log.timestamp && (
                  <span className={styles.logTime}>
                    {new Date(log.timestamp).toLocaleTimeString('ru-RU')}
                  </span>
                )}
                <span className={`${styles.levelBadge} ${getLevelClass(log.level)}`}>
                  {log.level}
                </span>
                {log.target && (
                  <span style={{ color: '#818cf8', fontWeight: 600 }}>[{log.target}]</span>
                )}
                <span className={styles.logMessage}>{log.message}</span>
              </div>
            ))}
          </div>
        )}

        {files.length > 0 && (
          <div style={{ padding: '12px 20px', fontSize: '0.8rem', color: '#94a3b8', borderTop: '1px solid rgba(255,255,255,0.05)' }}>
            Файлы логов в системе: {files.join(', ')}
          </div>
        )}
      </div>

      {/* Log JSON Details Modal */}
      {selectedLog && (
        <div className={styles.modalBackdrop} onClick={() => setSelectedLog(null)}>
          <div className={styles.modal} style={{ maxWidth: 640 }} onClick={(e) => e.stopPropagation()}>
            <h3 className={styles.modalTitle}>Детали лог-записи</h3>
            <pre
              style={{
                background: '#090d16',
                padding: 16,
                borderRadius: 8,
                overflowX: 'auto',
                fontSize: '0.8rem',
                color: '#34d399',
                maxHeight: 360,
              }}
            >
              {JSON.stringify(selectedLog.raw, null, 2)}
            </pre>
            <div className={styles.modalActions}>
              <button className={styles.cancelBtn} onClick={() => setSelectedLog(null)}>
                Закрыть
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}

function getLevelClass(level: string) {
  switch (level.toUpperCase()) {
    case 'WARN':
    case 'WARNING':
      return styles.levelWARN
    case 'ERROR':
      return styles.levelERROR
    default:
      return styles.levelINFO
  }
}

function SearchIcon() {
  return (
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
      <circle cx="11" cy="11" r="7" stroke="currentColor" strokeWidth="2" />
      <path d="m16.5 16.5 4 4" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
    </svg>
  )
}

function RefreshIcon() {
  return (
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
      <path d="M23 4v6h-6" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
      <path d="M1 20v-6h6" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
      <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  )
}
