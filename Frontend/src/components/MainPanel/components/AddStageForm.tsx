import React, { useState } from 'react'
import { useAppendStageMutation, useInsertStageMutation } from '../../../store/crmApi'
import { SendIcon } from './Helpers'
import styles from '../MainPanel.module.scss'

interface AddStageFormProps {
  projectId: string
}

export default function AddStageForm({ projectId }: AddStageFormProps) {
  const [title, setTitle]       = useState('')
  const [position, setPosition] = useState('')
  const [error, setError]       = useState<string | null>(null)

  const [appendStage, { isLoading: appending }] = useAppendStageMutation()
  const [insertStage, { isLoading: inserting }] = useInsertStageMutation()
  const creating = appending || inserting

  const handleSend = async () => {
    const t = title.trim()
    const p = position.trim()
    if (!t || creating) return

    setError(null)

    try {
      if (p === '') {
        await appendStage({ projectId, title: t }).unwrap()
      } else {
        const parsedPos = parseInt(p, 10)
        if (isNaN(parsedPos) || parsedPos < 1) {
          setError('Номер этапа должен быть целым числом от 1')
          return
        }
        await insertStage({ projectId, position: parsedPos, title: t }).unwrap()
      }
      setTitle('')
      setPosition('')
    } catch (err) {
      console.error('Failed to create stage:', err)
      setError('Не удалось создать этап. Проверьте подключение к серверу.')
    }
  }

  const canSend = title.trim() !== '' && !creating

  return (
    <div className={styles.addStageWrapper}>
      {error && <div className={styles.errorMessage}>{error}</div>}
      <div className={styles.inputRow}>
        <input
          className={styles.posInput}
          type="number"
          placeholder="№"
          min={1}
          value={position}
          onChange={(e) => { setPosition(e.target.value); setError(null) }}
          onKeyDown={(e) => {
            if (e.key === 'Enter') {
              e.preventDefault()
              handleSend()
            }
          }}
        />
        <input
          className={styles.textInput}
          placeholder="Новый этап…"
          value={title}
          onChange={(e) => { setTitle(e.target.value); setError(null) }}
          onKeyDown={(e) => {
            if (e.key === 'Enter') {
              e.preventDefault()
              handleSend()
            }
          }}
        />
        <button className={styles.sendBtn} onClick={handleSend} disabled={!canSend}>
          <SendIcon />
        </button>
      </div>
    </div>
  )
}
