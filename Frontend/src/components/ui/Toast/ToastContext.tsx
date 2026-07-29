import React, { createContext, useContext, useState, useCallback, type ReactNode } from 'react'
import { CloseIcon } from '../icons'
import styles from './Toast.module.scss'

export type ToastType = 'error' | 'success' | 'info'

interface ToastMessage {
  id: string
  text: string
  type: ToastType
}

interface ToastContextValue {
  showToast: (text: string, type?: ToastType) => void
}

const ToastContext = createContext<ToastContextValue | undefined>(undefined)

export function ToastProvider({ children }: { children: ReactNode }) {
  const [toasts, setToasts] = useState<ToastMessage[]>([])

  const removeToast = useCallback((id: string) => {
    setToasts((prev) => prev.filter((t) => t.id !== id))
  }, [])

  const showToast = useCallback((text: string, type: ToastType = 'error') => {
    const id = `${Date.now()}-${Math.random()}`
    setToasts((prev) => [...prev, { id, text, type }])
    setTimeout(() => {
      removeToast(id)
    }, 4000)
  }, [removeToast])

  return (
    <ToastContext.Provider value={{ showToast }}>
      {children}
      <div className={styles.toastContainer}>
        {toasts.map((toast) => {
          const typeClass =
            toast.type === 'error'
              ? styles.toastError
              : toast.type === 'success'
              ? styles.toastSuccess
              : styles.toastInfo
          return (
            <div key={toast.id} className={`${styles.toast} ${typeClass}`}>
              <span>{toast.text}</span>
              <button className={styles.closeBtn} onClick={() => removeToast(toast.id)}>
                <CloseIcon size={12} />
              </button>
            </div>
          )
        })}
      </div>
    </ToastContext.Provider>
  )
}

export function useToast() {
  const context = useContext(ToastContext)
  if (!context) {
    throw new Error('useToast must be used within a ToastProvider')
  }
  return context
}
