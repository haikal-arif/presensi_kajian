Write-Output 'CREATE TABLE "presensi" ('
Write-Output '	"id"	INTEGER NOT NULL,'
Write-Output '	"nama"	TEXT,'

for (($i = 0); $i -lt 100; $i++)
{
    Write-Output "	`"column$i`" TEXT,"
}

Write-Output "	PRIMARY KEY("id" AUTOINCREMENT)"
Write-Output ");"
