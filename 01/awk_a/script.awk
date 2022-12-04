BEGIN{
    print("===========================");
    print("  Advent Of Code 2022");
    print("  Day 01");
    print("  Awk version");
    print("===========================");
    max=0;
    sum=0
    }
{
if($1==""){if(sum>max){max=sum};sum=0} else {sum=sum+$1}
}
END{print max}
