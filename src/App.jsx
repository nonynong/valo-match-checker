import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import MatchCard from './components/MatchCard'
import './App.css'

const THEME_KEY = 'valorant-menubar-theme'

function App() {
  const [matches, setMatches] = useState([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState(null)
  const [currentIndex, setCurrentIndex] = useState(0)
  const [theme, setTheme] = useState(() => {
    try {
      return localStorage.getItem(THEME_KEY) || 'dark'
    } catch {
      return 'dark'
    }
  })

  useEffect(() => {
    document.documentElement.setAttribute('data-theme', theme)
    try {
      localStorage.setItem(THEME_KEY, theme)
    } catch {}
  }, [theme])

  const toggleTheme = () => setTheme((t) => (t === 'dark' ? 'light' : 'dark'))

  const fetchMatches = async () => {
    try {
      setLoading(true)
      setError(null)
      const result = await invoke('get_live_matches')
      setMatches(result || [])
      setCurrentIndex((i) => (result?.length ? Math.min(i, result.length - 1) : 0))
    } catch (err) {
      console.error('Error fetching matches:', err)
      setError(err.message || 'Failed to load matches')
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    fetchMatches()
    const interval = setInterval(fetchMatches, 30000)

    const handleBlur = async () => {
      setTimeout(async () => {
        try {
          const window = getCurrentWindow()
          const isFocused = await window.isFocused()
          if (!isFocused) {
            await window.hide()
          }
        } catch (err) {}
      }, 100)
    }

    window.addEventListener('blur', handleBlur)
    return () => {
      clearInterval(interval)
      window.removeEventListener('blur', handleBlur)
    }
  }, [])

  if (loading && matches.length === 0) {
    return (
      <div className="app-container">
        <div className="loading-state">
          <div className="loading-spinner"></div>
          <p>Loading live matches...</p>
        </div>
      </div>
    )
  }

  if (error && matches.length === 0) {
    return (
      <div className="app-container">
        <div className="error-state">
          <p>⚠️ {error}</p>
          <button onClick={fetchMatches} className="retry-button">
            Retry
          </button>
        </div>
      </div>
    )
  }

  const match = matches[currentIndex]
  const hasMultiple = matches.length > 1
  const canGoPrev = hasMultiple && currentIndex > 0
  const canGoNext = hasMultiple && currentIndex < matches.length - 1

  return (
    <div className="app-container">
      <div className="app-header">
        <div className="header-content">
          <div className="game-pill">
            <span className="game-pill-dot"></span>
            <span>Valorant</span>
          </div>
          <div className="header-text">
            <h1 className="title">Live Match</h1>
          </div>
        </div>
        <div className="header-actions">
          <button
            type="button"
            className="theme-toggle icon-button"
            onClick={toggleTheme}
            title={theme === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'}
            aria-label={theme === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'}
          >
            {theme === 'dark' ? (
              <svg className="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                <circle cx="12" cy="12" r="5" />
                <path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42" />
              </svg>
            ) : (
              <svg className="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" />
              </svg>
            )}
          </button>
          <button type="button" onClick={fetchMatches} className="refresh-button icon-button" title="Refresh" aria-label="Refresh">
            <svg className="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <path d="M23 4v6h-6" />
              <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10" />
            </svg>
          </button>
        </div>
      </div>

      <div className="matches-list">
        {matches.length === 0 ? (
          <div className="empty-state">
            <p>No live matches at the moment</p>
          </div>
        ) : (
          <MatchCard match={match} />
        )}
      </div>

      {hasMultiple && (
        <div className="match-nav">
          <button
            type="button"
            className="nav-arrow"
            onClick={() => setCurrentIndex((i) => Math.max(0, i - 1))}
            disabled={!canGoPrev}
            aria-label="Previous match"
          >
            ←
          </button>
          <span className="match-counter">
            {currentIndex + 1} / {matches.length}
          </span>
          <button
            type="button"
            className="nav-arrow"
            onClick={() => setCurrentIndex((i) => Math.min(matches.length - 1, i + 1))}
            disabled={!canGoNext}
            aria-label="Next match"
          >
            →
          </button>
        </div>
      )}

      {matches.length > 0 && (
        <div className="app-footer">
          <span className="footer-text">
            {matches.length} live match{matches.length === 1 ? '' : 'es'}
          </span>
        </div>
      )}
    </div>
  )
}

export default App
