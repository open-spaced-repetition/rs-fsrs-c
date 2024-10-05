#include "../include/fsrs.h"
#include <stdint.h>
#include <stdio.h>
#include <time.h>

int main(void) {
  fsrs_Fsrs F1 = fsrs_Fsrs_new(
      (fsrs_Parameters){.maximum_interval = 36000,
                        .request_retention = 0.9,
                        .w = {0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                              0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0}});
  fsrs_Card c = fsrs_Card_new();
  fsrs_Fsrs F = fsrs_Fsrs_default();
  time_t t = time(NULL);

  fsrs_ScheduledCards scheduled_cards = fsrs_schedule_timestamp(&F, &c, t);
  fsrs_Rating r = Easy;
  fsrs_Card updated_card = select_card(&scheduled_cards, r);
  fsrs_ReviewLog log = fsrs_get_ReviewLog(&updated_card);

  struct tm *lt = localtime(&log.reviewed_date_s);
  printf("scheduled_days: %ld, elapsed_days: %ld, timestamp: %ld, rating: "
         "%d, state: %d\n",
         log.scheduled_days, log.elapsed_days, log.reviewed_date_s, log.rating,
         log.state);

  puts(asctime(lt));
  return 0;
}