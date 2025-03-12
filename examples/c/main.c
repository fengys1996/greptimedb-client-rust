#include "client.h"
#include <stdio.h>
#include <unistd.h>

int main()
{
	ClientOptions client_options = {
		.endpoint = "127.0.0.1:4001",
		.db_name = "public",
		.secure = false,
	};

	Client * client_ptr = NULL;
	if (0 != client_new(client_options, &client_ptr) ) {
		printf("failed to create client\n");
		return 1;
	}

	printf("Client is created\n");

	const char * table_name = "monitor";

	ColumnSchema id = {
		.name = "id",
		.data_type = I32,
		.semantic_type = Tag,
	};

	ColumnSchema cpu = {
		.name = "cpu",
		.data_type = F32,
		.semantic_type = Field,
	};

	ColumnSchema ts = {
		.name = "ts",
		.data_type = TimestampMillisecond,
		.semantic_type = Timestamp,
	};

	ColumnSchema column_schemas[] = {id, cpu, ts};
	size_t columns_len = 3;

	size_t capacity = 1;

	RowBatch * row_batch_ptr = NULL;
	if(0 != row_batch_new_with_capacity(table_name, capacity, column_schemas, columns_len, &row_batch_ptr)) {
		printf("failed to create rows\n");
		client_free(client_ptr);
		return 1;
	}

	printf("Rows created\n");

	Value id_val = {
		.tag = I32Value,
		.i32_value = 1,
	};
	
	Value cpu_val = {
		.tag = F32Value,
		.f32_value = 11.1,
	};

	Value ts_val = {
		.tag = TimestampValueMillisecond,
		.i64_value = 1741775386000,
	};

	Value values[] = {id_val, cpu_val, ts_val};
	size_t value_len = 3;

	if(0 != row_batch_push_row(values, value_len, row_batch_ptr)) {
		printf("failed to push row\n");
		row_batch_free(row_batch_ptr);
		client_free(client_ptr);
		return 1;
	}

	printf("Row pushed\n");

	if(0 != client_insert_row_batch(row_batch_ptr, client_ptr)) {
		printf("failed to insert rows\n");
		row_batch_free(row_batch_ptr);
		client_free(client_ptr);
		return 1;
	}
	printf("Rows inserted\n");

	row_batch_free(row_batch_ptr);
	client_free(client_ptr);
	
	return 0;
}
