package audit

import (
	"fmt"
	"log"
	"os"
	"sync"
	"time"
)

type EventType string

const (
	EventCertIssued  EventType = "CERTIFICATE_ISSUED"
	EventCertRevoked EventType = "CERTIFICATE_REVOKED"
	EventCertExpired EventType = "CERTIFICATE_EXPIRED"
	EventIdentityReg EventType = "IDENTITY_REGISTERED"
	EventAuthSuccess EventType = "AUTHENTICATION_SUCCESS"
	EventAuthFailure EventType = "AUTHENTICATION_FAILURE"
	EventAccessDenied EventType = "ACCESS_DENIED"
	EventConfigChange EventType = "CONFIGURATION_CHANGED"
)

type Event struct {
	ID        string    `json:"id"`
	Timestamp time.Time `json:"timestamp"`
	Type      EventType `json:"type"`
	Actor     string    `json:"actor"`
	Action    string    `json:"action"`
	Resource  string    `json:"resource"`
	Result    string    `json:"result"`
	Details   string    `json:"details"`
}

type Logger struct {
	mu     sync.Mutex
	events []Event
	logger *log.Logger
}

func NewLogger() *Logger {
	return &Logger{
		events: make([]Event, 0),
		logger: log.New(os.Stdout, "[AUDIT] ", log.LstdFlags),
	}
}

func (l *Logger) Log(event Event) {
	l.mu.Lock()
	defer l.mu.Unlock()

	event.ID = fmt.Sprintf("evt-%d", len(l.events)+1)
	event.Timestamp = time.Now().UTC()
	l.events = append(l.events, event)
	l.logger.Printf("%s | %s | %s | %s | %s",
		event.Type, event.Actor, event.Action, event.Resource, event.Result)
}

func (l *Logger) Query(filter EventFilter) []Event {
	l.mu.Lock()
	defer l.mu.Unlock()

	var result []Event
	for _, e := range l.events {
		if filter.Type != "" && e.Type != filter.Type {
			continue
		}
		if filter.Actor != "" && e.Actor != filter.Actor {
			continue
		}
		if filter.Limit > 0 && len(result) >= filter.Limit {
			break
		}
		result = append(result, e)
	}
	return result
}

type EventFilter struct {
	Type  EventType
	Actor string
	Limit int
}
