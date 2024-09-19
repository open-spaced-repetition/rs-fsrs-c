#include "../include/fsrs.h"
#include <stdint.h>
#include <stdio.h>
#include <time.h>

int main(void) {
  fsrs_fsrs F = fsrs_fsrs_new();
  fsrs_card c = fsrs_card_new();
  time_t t = time(NULL);

  fsrs_ScheduledCards scheduled_cards = fsrs_schedule_s(&F, &c, t);
  fsrs_Rating r = Easy;
  fsrs_card updated_card = select_card(&scheduled_cards, r);
  fsrs_ReviewLog log = fsrs_ReviewLog_log(&updated_card);

  struct tm *lt = localtime(&log.reviewed_date_s);
  char arr[200];
  printf("scheduled_days: %ld, elapsed_days: %ld, timestamp: %ld, rating: "
         "%d, state: %d\n",
         log.scheduled_days, log.elapsed_days, log.reviewed_date_s, log.rating,
         log.state);

  printf("%dy %dm %dd %dh %dm %ds\n", lt->tm_year + 1900, lt->tm_mon + 1,
         lt->tm_mday, lt->tm_hour, lt->tm_min, lt->tm_sec);
  return 0;
}