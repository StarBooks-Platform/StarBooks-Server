using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace StarBooks.Infrastructure.Migrations
{
    public partial class ChangeAverageRatingTypeToDouble : Migration
    {
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.AlterColumn<double>(
                name: "VolumeInfo_AverageRating",
                table: "books",
                type: "float",
                nullable: false,
                oldClrType: typeof(decimal),
                oldType: "decimal(18,2)");
        }

        protected override void Down(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.AlterColumn<decimal>(
                name: "VolumeInfo_AverageRating",
                table: "books",
                type: "decimal(18,2)",
                nullable: false,
                oldClrType: typeof(double),
                oldType: "float");
        }
    }
}
