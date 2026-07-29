import React, { useState } from 'react'
import { useCreateInviteMutation } from '../../../store/crmApi'
import styles from '../UserPage.module.scss'

export default function UserInvitesSection() {
  const [inviteToken, setInviteToken] = useState<string | null>(null)
  const [copied, setCopied] = useState(false)
  const [createInvite, { isLoading: creatingInvite }] = useCreateInviteMutation()

  const handleGenerateInvite = async () => {
    setInviteToken(null)
    setCopied(false)
    const result = await createInvite()
    if ('data' in result && result.data) setInviteToken(result.data.token)
  }

  const inviteLink = inviteToken ? `${window.location.origin}/?invite=${inviteToken}` : null

  const handleCopy = () => {
    if (!inviteLink) return
    navigator.clipboard.writeText(inviteLink)
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  return (
    <section className={styles.section}>
      <h2 className={styles.sectionTitle}>Приглашение</h2>
      <p className={styles.current}>Токен действует 7 дней</p>
      <button className={styles.btn} onClick={handleGenerateInvite} disabled={creatingInvite}>
        {creatingInvite ? '…' : 'Создать приглашение'}
      </button>
      {inviteLink && (
        <div className={styles.tokenBox}>
          <span className={styles.tokenText}>{inviteLink}</span>
          <button className={styles.copyBtn} onClick={handleCopy}>
            {copied ? <CheckIcon /> : <CopyIcon />}
          </button>
        </div>
      )}
    </section>
  )
}

function CopyIcon() {
  return (
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none">
      <rect x="9" y="9" width="13" height="13" rx="2" stroke="currentColor" strokeWidth="2" />
      <path
        d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
      />
    </svg>
  )
}

function CheckIcon() {
  return (
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none">
      <path d="M20 6 9 17l-5-5" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  )
}
