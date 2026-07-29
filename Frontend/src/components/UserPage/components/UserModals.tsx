import React, { useState } from 'react'
import FormModal from '../../FormModal/FormModal'
import {
  useUpdateUsernameMutation,
  useUpdatePasswordMutation,
  useUpdateEmailMutation,
} from '../../../store/crmApi'
import styles from '../UserPage.module.scss'

type ModalKind = 'username' | 'email' | 'password'

interface UserModalsProps {
  modal: ModalKind | null
  onClose: () => void
}

export default function UserModals({ modal, onClose }: UserModalsProps) {
  const [username, setUsername] = useState('')
  const [usernameError, setUsernameError] = useState<string | null>(null)

  const [email, setEmail] = useState('')
  const [emailError, setEmailError] = useState<string | null>(null)

  const [currentPassword, setCurrentPassword] = useState('')
  const [newPassword, setNewPassword] = useState('')
  const [confirmPassword, setConfirmPassword] = useState('')
  const [passwordError, setPasswordError] = useState<string | null>(null)

  const [updateUsername, { isLoading: savingUsername }] = useUpdateUsernameMutation()
  const [updatePassword, { isLoading: savingPassword }] = useUpdatePasswordMutation()
  const [updateEmail, { isLoading: savingEmail }] = useUpdateEmailMutation()

  const handleClose = () => {
    setUsernameError(null)
    setEmailError(null)
    setPasswordError(null)
    onClose()
  }

  const handleUsernameSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setUsernameError(null)
    const result = await updateUsername({ username: username.trim() })
    if ('error' in result) {
      const status = (result.error as { status?: number })?.status
      setUsernameError(status === 409 ? 'Имя пользователя уже занято' : 'Что-то пошло не так')
    } else {
      setUsername('')
      handleClose()
    }
  }

  const handleEmailSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setEmailError(null)
    const result = await updateEmail({ email: email.trim() })
    if ('error' in result) {
      const status = (result.error as { status?: number })?.status
      setEmailError(status === 409 ? 'Этот email уже используется' : 'Что-то пошло не так')
    } else {
      setEmail('')
      handleClose()
    }
  }

  const handlePasswordSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setPasswordError(null)
    if (newPassword !== confirmPassword) {
      setPasswordError('Пароли не совпадают')
      return
    }
    const result = await updatePassword({ current_password: currentPassword, new_password: newPassword })
    if ('error' in result) {
      const status = (result.error as { status?: number })?.status
      setPasswordError(status === 401 ? 'Неверный текущий пароль' : 'Что-то пошло не так')
    } else {
      setCurrentPassword('')
      setNewPassword('')
      setConfirmPassword('')
      handleClose()
    }
  }

  return (
    <>
      {modal === 'username' && (
        <FormModal
          heading="Изменить имя пользователя"
          onClose={handleClose}
          onSubmit={handleUsernameSubmit}
          loading={savingUsername}
          error={usernameError}
          submitLabel="Сохранить"
        >
          <input
            className={styles.modalInput}
            placeholder="Новое имя пользователя"
            value={username}
            onChange={(e) => {
              setUsername(e.target.value)
              setUsernameError(null)
            }}
            autoComplete="username"
            autoFocus
            required
          />
        </FormModal>
      )}

      {modal === 'email' && (
        <FormModal
          heading="Изменить email"
          onClose={handleClose}
          onSubmit={handleEmailSubmit}
          loading={savingEmail}
          error={emailError}
          submitLabel="Сохранить"
        >
          <input
            className={styles.modalInput}
            type="email"
            placeholder="Новый email"
            value={email}
            onChange={(e) => {
              setEmail(e.target.value)
              setEmailError(null)
            }}
            autoComplete="email"
            autoFocus
            required
          />
        </FormModal>
      )}

      {modal === 'password' && (
        <FormModal
          heading="Изменить пароль"
          onClose={handleClose}
          onSubmit={handlePasswordSubmit}
          loading={savingPassword}
          error={passwordError}
          submitLabel="Изменить"
        >
          <input
            className={styles.modalInput}
            type="password"
            placeholder="Текущий пароль"
            value={currentPassword}
            onChange={(e) => {
              setCurrentPassword(e.target.value)
              setPasswordError(null)
            }}
            autoComplete="current-password"
            autoFocus
            required
          />
          <input
            className={styles.modalInput}
            type="password"
            placeholder="Новый пароль"
            value={newPassword}
            onChange={(e) => {
              setNewPassword(e.target.value)
              setPasswordError(null)
            }}
            autoComplete="new-password"
            required
          />
          <input
            className={styles.modalInput}
            type="password"
            placeholder="Повторите новый пароль"
            value={confirmPassword}
            onChange={(e) => {
              setConfirmPassword(e.target.value)
              setPasswordError(null)
            }}
            autoComplete="new-password"
            required
          />
        </FormModal>
      )}
    </>
  )
}
