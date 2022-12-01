BEGIN {max=0;sum=0} {if($1=="") { if(sum>max){max=sum};sum=0} else {sum=sum+$1}} END {print max}
