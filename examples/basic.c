#include "../include/fsrs.h"
#include <stdint.h>
#include <stdio.h>

int main(void) {
  fsrs_fsrs F = fsrs_fsrs_new();
  fsrs_card c = fsrs_card_new();
  fsrs_ScheduledCards scheduled_cards = fsrs_schedule(&F, &c);
  fsrs_Rating r = Easy;
  fsrs_card updated_card = select_card(&scheduled_cards, r);
  printf("scheduled_days: %ld\n", fsrs_schedule_debug(&updated_card));
  return 0;
}