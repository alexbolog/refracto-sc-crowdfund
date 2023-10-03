mod basic_state_transition;
mod specific_state_transitions;
// thesis
/*
   Invalid = 0,
   Pending = 1, => prior starting
   CFActive = 2, => crowdfunding active - in between start and end timestamps, investment target not reached
   CFWaitingCooloff = 3, => crowdfunding active - in between start and end timestamps, investment target reached but partially in cool off
   CFSuccessful = 4, => crowdfunding active - in between start and end timestamps, investment target reached
   CFFailed = 5, => crowdfunding active - at end timestamp, investment target not reached
   CFCancelled = 6, => admin action
   LoanActive = 7, => withdrawn funds, awaiting repayment
   Completed = 8, => loan repayment completed, awaiting investor claim
*/
